# css/selectors/old-tests/css3-modsel-44c.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-44c.xml"
}
```

## style[0]

```css
<![CDATA[
  .fail > div { background: red; color: yellow; }
  .control { background: green; color: white; }
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
