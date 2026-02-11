# css/selectors/selectors-4/lang-021.html

```json
{
  "format_version": 3,
  "file": "css/selectors/selectors-4/lang-021.html"
}
```

## style[0]

```css

div.test { color: red; }
span:lang("en-gb-oed") { color: magenta; }
span span:lang("*-gb") { color: green; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
