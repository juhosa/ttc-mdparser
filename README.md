# Thing To Check Markdown Parser

This app just for reading a very specific markdown file with the following format:

```markdown
## Videos

- Item 1

## Links

- Link 1

## Things to check

> Move to a correct place after review.

- [Link text](URL)
```

The list after the "Things to check" heading is being read and printed out.

This was mostly an excercise in parsing the AST generated from the markdown.

## Checklist

- [x] Overly complicated parsing of a simple file
- [x] Totally unnecessary app overall
- [x] Learned a lot while writing it
