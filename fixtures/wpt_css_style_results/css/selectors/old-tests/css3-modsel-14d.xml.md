# css/selectors/old-tests/css3-modsel-14d.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-14d.xml"
}
```

## style[0]

```css
<![CDATA[
p { background: green; color: white; }
.t1:not(.t2) { background: red; color: yellow; }
:not(.t2).t1 { background: red; color: yellow; }
.t2:not(.t1) { background: red; color: yellow; }
:not(.t1).t2 { background: red; color: yellow; }
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
