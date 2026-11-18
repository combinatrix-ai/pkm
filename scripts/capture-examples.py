"""Regenerate website examples from a disposable synthetic vault.

Every `result` on the landing page is real output from the binary built by
`cargo build --bin pkm`. The questions and the agent's replies are written
here by hand; keep them consistent with the captured output.
"""
import json
from pathlib import Path
import shlex
import subprocess
import tempfile

root = Path(__file__).resolve().parent.parent
binary = root / 'target/debug/pkm'

BASE = {
    'pkm.yaml': 'vault_path: .\nlint:\n  rules:\n    orphans: {enabled: false}\n',
    'Index.md': '# Index\n\n- [[Projects/Redesign]]\n- [[Meetings/Kickoff]]\n',
    'Projects/Redesign.md': '---\ntype: project\nstatus: active\n---\n# Redesign\n\nScope agreed in [[Meetings/Kickoff]].\n',
    'Meetings/Kickoff.md': '# Kickoff\n\nFirst meeting for [the redesign](../Projects/Redesign.md).\n',
    'Diary/2026-01-12.md': '# 2026-01-12\n\nDrafted the new homepage for [[Projects/Redesign]].\n',
    'Projects/Tasks.md': '# Tasks\n\n- [ ] Send the proposal 📅 2026-01-15\n- [ ] Book the user sessions 📅 2026-01-20\n',
}

RULES = '''lint:
  paths:
    - path: ^Projects/
      where: ['type=project']
      headings:
        required: [Summary, Next steps]
'''

# name -> (extra files, argv, expected exit code)
CASES = {
    'rename': ({}, ['rename', 'Redesign', 'Relaunch'], 0),
    'tasks': ({}, ['tasks', '--overdue'], 0),
    'backlinks': ({}, ['backlinks', 'Projects/Redesign.md'], 0),
    'broken': ({'Projects/Pricing.md': '# Pricing\n\nCompare with [[Competitor notes]].\n'}, ['broken-links'], 0),
    'repair': (
        {'Index.md': '# Index\n\n- [[old/Meetings/Kickoff.md]]\n- [[Projects/My%20Plan]]\n- [[Roam]]\n',
         'Projects/My Plan.md': '# My Plan\n'},
        ['repair-links', '--dry-run'], 0),
    'demote': (
        {'Diary/2026-01-12.md': '# 2026-01-12\n\n- The piece on pricing pages [[to-read]]\n- Asked Sam about [[to-read|the reading list]]\n',
         'Diary/2026-01-13.md': '# 2026-01-13\n\n- Two papers on onboarding [[to-read]]\n'},
        ['demote', 'to-read', '--dry-run'], 0),
    'movedir': ({}, ['move', 'Projects', 'Archive/2025/'], 0),
    'context': ({}, ['context', 'Projects/Redesign.md', '--format', 'json'], 0),
    'rules': ({'pkm.yaml': 'vault_path: .\n' + RULES}, ['lint', '--check', 'headings'], 1),
    'refs': (
        {'Projects/Application.md': '# Application\n\nAddress: [Home address](pkm://personal/address.home)\n'},
        ['refs', 'Projects/Application.md'], 0),
}


def capture(name):
    extra, args, expected = CASES[name]
    with tempfile.TemporaryDirectory(prefix='pkm-website-') as directory:
        vault = Path(directory)
        for relative, text in {**BASE, **extra}.items():
            (vault / relative).parent.mkdir(parents=True, exist_ok=True)
            (vault / relative).write_text(text)
        result = subprocess.run(
            [str(binary), *args], cwd=vault, capture_output=True, text=True,
            env={'PATH': '/usr/bin:/bin', 'HOME': directory, 'PKM_NO_UPDATE_CHECK': '1'})
        if result.returncode != expected:
            raise RuntimeError(f'{name}: exit {result.returncode}\n{result.stderr or result.stdout}')
        return {'command': 'pkm ' + shlex.join(args), 'result': result.stdout.replace(directory, '<vault>').strip()}


conversations = [
    {'name': 'Rename', 'case': 'rename',
     'you': 'Rename the Redesign project to Relaunch.',
     'agent': 'Renamed. The three notes that linked to it now point to Relaunch.',
     'caption': 'A rename that carries every link with it.'},
    {'name': 'Tasks', 'case': 'tasks',
     'you': "What's overdue?",
     'agent': 'Two things: send the proposal (Jan 15) and book the user sessions (Jan 20). Want me to move either one?',
     'caption': 'Tasks from every note, filtered by date, status, or tag.'},
    {'name': 'Recall', 'case': 'backlinks',
     'you': 'Where have I mentioned the redesign?',
     'agent': 'Three places: your index, the kickoff meeting note, and the Jan 12 diary entry.',
     'caption': 'The backlinks pane, as an answer.'},
    {'name': 'Health', 'case': 'broken',
     'you': 'Is anything broken?',
     'agent': 'One broken link: Projects/Pricing.md points at "Competitor notes", which does not exist. Should I create it or fix the link?',
     'caption': 'Every broken link, with the line and the reason.'},
]

examples = {
    'conversations': [
        {**{k: c[k] for k in ('name', 'you', 'agent', 'caption')}, **capture(c['case'])} for c in conversations
    ],
    'snippets': {name: capture(name) for name in ('rename', 'repair', 'demote', 'rules', 'refs', 'movedir', 'context')},
    'rules_config': RULES.strip(),
}
(root / 'website/demo/examples.json').write_text(json.dumps(examples, ensure_ascii=False, indent=2) + '\n')
