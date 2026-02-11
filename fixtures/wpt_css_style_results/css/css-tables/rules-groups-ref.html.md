# css/css-tables/rules-groups-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/rules-groups-ref.html"
}
```

## style[0]

```css

table { margin: 1em 1em 2em 1em; border-collapse: collapse; }

#a thead, #a tbody { border-block-end-width: 1px; border-block-end-style: solid; }
#b thead, #b tbody { border-block-end: 1px solid blue; }
#c thead, #c tbody { border-block-end-width: 5px; border-block-end-style: solid; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-block-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
