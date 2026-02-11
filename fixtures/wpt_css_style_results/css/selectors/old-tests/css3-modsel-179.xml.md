# css/selectors/old-tests/css3-modsel-179.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-179.xml"
}
```

## style[0]

```css
<![CDATA[
  p { color: green; }
  span:first-line { background: red; color: yellow; font-size: 4em; }
  span::first-line { background: red; color: yellow; font-size: 4em; }
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
