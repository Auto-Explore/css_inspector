# css/selectors/old-tests/css3-modsel-45c.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-45c.xml"
}
```

## style[0]

```css
<![CDATA[
  .fail + div { background: red; }
  .control { background: lime; }
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
