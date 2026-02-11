# css/selectors/old-tests/css3-modsel-43b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-43b.xml"
}
```

## style[0]

```css
<![CDATA[.white { background-color: transparent ! important; }
.green { background-color: lime; }
div.t1 p { background-color: red; }]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
