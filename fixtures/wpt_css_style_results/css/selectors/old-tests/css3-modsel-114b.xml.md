# css/selectors/old-tests/css3-modsel-114b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-114b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|address, *|q, *|r { display : block ; margin-bottom : 1em }
*|p, *|q { background-color : lime ! important }
*|*[|lang|="foo-bar"], *|*[|myattr|="tat-tut"] { background-color : red }]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
