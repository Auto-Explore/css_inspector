# css/selectors/old-tests/css3-modsel-97b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-97b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
*|p, *|q, *|r { display : block ; margin-bottom : 1em }
*|q { background-color : lime ! important }
*[a|title] {background-color : red }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
