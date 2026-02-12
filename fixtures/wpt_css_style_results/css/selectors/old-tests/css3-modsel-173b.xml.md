# css/selectors/old-tests/css3-modsel-173b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-173b.xml"
}
```

## style[0]

```css
<![CDATA[
 tests, tests * { display: block; color: red; }
 testA[*|attribute] { color: green; }
 testB[*|attribute="pass"] { color: green; }
 testC[*|attribute~="pass"] { color: green; }
 testD[*|attribute^="pass"] { color: green; }
 testE[*|attribute*="pass"] { color: green; }
 testF[*|attribute$="pass"] { color: green; }
 testG[*|attribute|="pass"] { color: green; }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
