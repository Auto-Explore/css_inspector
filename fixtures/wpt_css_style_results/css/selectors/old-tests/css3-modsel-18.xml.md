# css/selectors/old-tests/css3-modsel-18.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-18.xml"
}
```

## style[0]

```css
<![CDATA[p:hover { background-color : lime }
a:hover { background-color : lime }

tr:hover { background-color : green }
td:hover { background-color : lime }

table { border-spacing: 5px; }]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
