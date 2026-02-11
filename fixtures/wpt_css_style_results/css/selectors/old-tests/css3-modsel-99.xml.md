# css/selectors/old-tests/css3-modsel-99.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-99.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|p, *|r { background-color : red }
*|*[a|foo~="bar"], *|*[|class~="bar"] { background-color : lime }
*|*[html|class~="bar"] { background-color : red }
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
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
