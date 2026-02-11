# css/selectors/old-tests/css3-modsel-139b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-139b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|address, *|q, *|r { display : block ; margin-bottom : 1em }
*|address, *|r { background-color : lime ! important }
div.stub *|*:not([|class~="foo"]) { background-color : red }]]>
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
