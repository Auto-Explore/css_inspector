# css/selectors/old-tests/css3-modsel-57b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-57b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
*|p, *|q, *|r { display : block ; margin-bottom : 1em }
*|p, *|r { background-color : lime ! important }
div.stub *:not([a|title]) {background-color : red }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
