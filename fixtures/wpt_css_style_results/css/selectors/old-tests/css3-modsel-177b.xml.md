# css/selectors/old-tests/css3-modsel-177b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-177b.xml"
}
```

## style[0]

```css
<![CDATA[
 div { color: green; }
 p::first-child { color: yellow; background: red; }
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
