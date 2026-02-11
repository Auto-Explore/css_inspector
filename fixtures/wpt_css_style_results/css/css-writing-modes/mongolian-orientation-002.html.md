# css/css-writing-modes/mongolian-orientation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/mongolian-orientation-002.html"
}
```

## style[0]

```css

div {
  font-family: "Mongolian White"; /* Not required for the test to work,
but it helps make it look not ugly if you have the font installed.*/
  font-size: 2em;
  margin: 1em;
  writing-mode: vertical-lr;
  float: left;
}
div:nth-of-type(1) { text-orientation: mixed; }
div:nth-of-type(2) { text-orientation: sideways; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
