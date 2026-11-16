# Folder

Links by name, with the extension, to a note that lives in a folder:

- bare: [[Draft.md]]
- heading: [[Draft.md#Heading]]
- block: [[Draft.md^block]]
- alias: [[Draft.md|the draft]]
- embed: ![[Draft.md]]
- upper case: [[Draft.MD]]

Anchors are still checked:

- missing heading: [[Draft.md#Nope]]
- missing block: [[Draft.md^nope]]

A file whose name really ends in .md: [[Report.md.md]]

Names that only start like the note, which must stay broken:

- [[Draft.mdx]]
- [[Draft.png]]
- [[Draft.md.bak]]

One name, two folders, with and without the extension: [[Same]] [[Same.md]]
