# css/selectors/old-tests/css3-modsel-132b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-132b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|p.deu, *|q { background-color : lime ! important }
div.stub html|*:not([*|class~="deux"]),
   div.stub *|*:not(html|*):not([*|foo~="deux"]) { background-color : red }
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
