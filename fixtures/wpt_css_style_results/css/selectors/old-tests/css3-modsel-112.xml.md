# css/selectors/old-tests/css3-modsel-112.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-112.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s, *|t{ display : block ; margin-bottom : 1em }
*|p, *|r { background-color : red }
*|*[|title="si on chantait"] { background-color : lime }]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
