# css/selectors/old-tests/css3-modsel-14e.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-14e.xml"
}
```

## style[0]

```css
<![CDATA[
p { background: green; color: white; }
p:not(.t1):not(.t2) { background: red; color: yellow; }
div { background: red; color: yellow; }
div:not(.t1) { background: green; color: white; }
address { background: green; color: white; }
address:not(.t5):not(.t5) { background: red; color: yellow; }
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
