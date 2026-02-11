# css/selectors/old-tests/css3-modsel-182.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-182.xml"
}
```

## style[0]

```css
<![CDATA[
p { color: green; }
foo\:bar { background: red; color: yellow; }
]]>
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
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
