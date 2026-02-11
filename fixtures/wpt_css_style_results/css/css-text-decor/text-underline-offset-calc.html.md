# css/css-text-decor/text-underline-offset-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-offset-calc.html"
}
```

## style[0]

```css

div {
  display: flex;
  font-size: 22px;
}
.underline {
  text-decoration-color: green;
  text-decoration-line: underline;
  text-decoration-thickness: 15px;
  text-decoration-skip-ink: none;
}
.ref {
  text-underline-offset: 1em;
}
.test1 {
  text-underline-offset: calc(1em);
}
.test2 {
  text-underline-offset: calc(100%);
}
.test3 {
  text-underline-offset: calc(50% + 11px);
}
.test4 {
  text-underline-offset: calc(50% + 0.5em);
}
.test5 {
  text-underline-offset: calc(0.5em + 11px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
