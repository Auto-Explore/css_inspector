# css/selectors/old-tests/css3-modsel-168.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-168.xml"
}
```

## style[0]

```css
<![CDATA[
  span:before { background-color: red; content: 'FAILED'; }
  span::before { background-color: lime; content: 'PASSED'; }
]]>
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
