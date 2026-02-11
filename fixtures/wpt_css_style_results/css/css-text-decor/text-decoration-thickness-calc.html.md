# css/css-text-decor/text-decoration-thickness-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-calc.html"
}
```

## style[0]

```css

div {
  display: flex;
  font-family: Ahem;
  font-size: 8px;
}
.underline {
  text-decoration-color: green;
  text-decoration-line: underline;
  text-decoration-skip-ink: none;
}
.overline {
  text-decoration-color: green;
  text-decoration-line: overline;
  text-decoration-skip-ink: none;
}
.line-through {
  text-decoration-color: green;
  text-decoration-line: line-through;
  text-decoration-skip-ink: none;
}
.ref {
  text-decoration-thickness: 8px;
}
.test1 {
  text-decoration-thickness: calc(1em);
}
.test2 {
  text-decoration-thickness: calc(100%);
}
.test3 {
  text-decoration-thickness: calc(50% + 4px);
}
.test4 {
  text-decoration-thickness: calc(50% + 0.5em);
}
.test5 {
  text-decoration-thickness: calc(0.5em + 4px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
