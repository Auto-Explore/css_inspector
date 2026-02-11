# css/selectors/old-tests/css3-modsel-27b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-27b.xml"
}
```

## style[0]

```css
<![CDATA[* html { background-color: red; }
* :root { background-color: red; }
p { color: green; }]]>
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
