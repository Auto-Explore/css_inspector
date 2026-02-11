# css/selectors/old-tests/css3-modsel-43.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-43.xml"
}
```

## style[0]

```css
<![CDATA[.white { background-color: transparent ! important; }
.red { background-color: red; }
div.t1 p { background-color: lime; }]]>
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
