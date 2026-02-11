# css/selectors/old-tests/css3-modsel-107b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-107b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|p, *|r { background-color : lime ! important }
*|*[*|lang|="en"], *|*[a|foo|="un-d"] { background-color : red }
]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
