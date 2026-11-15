"""Regenerate website examples from a disposable synthetic vault."""
import json
from pathlib import Path
import shlex
import subprocess
import tempfile

root = Path(__file__).resolve().parent.parent
binary = root / 'target/debug/knapper'
examples = []
with tempfile.TemporaryDirectory(prefix='knapper-website-') as directory:
    vault = Path(directory)
    (vault / 'Templates').mkdir()
    (vault / 'Projects').mkdir()
    (vault / 'knapper.yaml').write_text('vault_path: .\nexclude: [Templates]\nlint:\n  rules:\n    orphans: {enabled: false}\n')
    (vault / 'Templates/Project.md').write_text('---\ntype: project\nstatus: active\n---\n# {{title}}\n\n## Summary\n\nA new research project.\n')
    (vault / 'Index.md').write_text('# Index\n\n[[Projects/Field notes]]\n')
    (vault / 'Projects/Review.md').write_text('# Review\n\n[[Missing note]]\n')
    (vault / 'Projects/Tasks.md').write_text('# Tasks\n\n- [ ] Review the field notes 📅 2026-01-15\n- [ ] Plan the next visit 📅 2026-01-20\n')
    cases = [
        ('Create', ['new', 'Field notes', '--template', 'Templates/Project.md', '--folder', 'Projects'], 'Start a note from your template, with its title and metadata in place.'),
        ('Query', ['query', '--where', 'status=active', '--format', 'paths'], 'Find notes by metadata, tags, and link counts.'),
        ('Links', ['backlinks', 'Projects/Field notes.md'], 'Follow the connections back to the notes that reference this one.'),
        ('Tasks', ['tasks', '--due-to', '2026-01-31'], 'Find tasks by dates, status, tags, or the notes they belong to.'),
        ('Lint', ['lint', 'Projects/Review.md', '--check', 'broken-links'], 'Catch broken links, missing headings, and invalid frontmatter.'),
        ('Refactor', ['rename', 'Field notes', 'Research notes', '--dry-run'], 'Preview a rename and its link updates before writing.'),
    ]
    for name, args, caption in cases:
        result = subprocess.run([str(binary), *args], cwd=vault, capture_output=True, text=True, env={'PATH': '/usr/bin:/bin', 'HOME': directory, 'KNAPPER_NO_UPDATE_CHECK': '1'})
        expected = 1 if name == 'Lint' else 0
        if result.returncode != expected:
            raise RuntimeError(result.stderr or result.stdout)
        output = result.stdout.replace(directory, '<vault>').strip()
        examples.append({'name': name, 'command': 'knapper ' + shlex.join(args), 'result': output, 'caption': caption})
(root / 'website/demo/examples.json').write_text(json.dumps(examples, ensure_ascii=False, indent=2) + '\n')
