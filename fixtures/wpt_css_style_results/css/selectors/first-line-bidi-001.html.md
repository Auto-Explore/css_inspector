# css/selectors/first-line-bidi-001.html

```json
{
  "format_version": 3,
  "file": "css/selectors/first-line-bidi-001.html"
}
```

## style[0]

```css

body { font: 12px arial, sans-serif; }
table { font-size: 10px; }
tr td { margin: 1em 1em; border: 1px solid gray; width: 16em; }
td:nth-child(1) { text-align: left; }
td:nth-child(2) { text-align: center; }
td:nth-child(3) { text-align: right; }
td p::first-line { color: green; background: pink; }
th { padding: 1em; }
td { padding: 2px; }
td p { margin: 0; }
xl { float: left; font-size: 2.5em; }
xr { float: right; font-size: 2.5em; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
