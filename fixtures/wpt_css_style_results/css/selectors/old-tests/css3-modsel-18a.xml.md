# css/selectors/old-tests/css3-modsel-18a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-18a.xml"
}
```

## style[0]

```css
<![CDATA[
p { color: navy; }

.a a:hover { background: green; color: white; }

.b a:hover { background: red; color: yellow; }
.b a:link { background: green; color: white; }

.c :link { background: green; color: white; }
.c :visited:hover { background: red; color: yellow; }
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
