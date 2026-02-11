# css/selectors/old-tests/css3-modsel-45b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-45b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color: lime; }
.white { background-color: transparent ! important; }
div.stub > p + p { background-color: red; }]]>
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
