# css/selectors/old-tests/css3-modsel-135b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-135b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s, *|t{ display : block ; margin-bottom : 1em }
*|p.red, *|q, *|t { background-color : lime ! important }
div.stub *|*:not([*|title$="tait"]) { background-color : red }]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
