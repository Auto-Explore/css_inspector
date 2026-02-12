# css/css-writing-modes/text-shadow-sideways-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-shadow-sideways-001-ref.html"
}
```

## style[0]

```css

body > div {
  /* styling */
  border: solid gray;
  font: 30px/1 Ahem;
  color: silver;
  float: left;
  margin: 8px;

  /* test */
  text-decoration: underline;
  text-decoration-color: orange;
  text-decoration-thickness: 14px;

  /* reference hacking */
  width: 1em;
  height: 5em;
  white-space: nowrap;
}
div > div {
  /* compensate for WebKit propagation bug */
  text-decoration-thickness: 4px;
}
.lr > div {
  text-shadow: -4px 2px aqua;
  transform: rotate(-90deg) translateX(-4em);
}
.rl > div {
  text-shadow: 4px -2px aqua;
  transform: rotate(90deg);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
