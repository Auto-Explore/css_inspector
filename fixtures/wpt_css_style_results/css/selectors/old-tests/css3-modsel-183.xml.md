# css/selectors/old-tests/css3-modsel-183.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-183.xml"
}
```

## style[0]

```css
<![CDATA[
p { color: green; }
..test { background: red; color: yellow; }
.foo..quux { background: red; color: yellow; }
.bar. { background: red; color: yellow; }
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
