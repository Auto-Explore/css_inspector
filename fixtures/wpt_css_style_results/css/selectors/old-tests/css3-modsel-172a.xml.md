# css/selectors/old-tests/css3-modsel-172a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-172a.xml"
}
```

## style[0]

```css
<![CDATA[
 tests, tests * { display: block; color: green; }
 testA[|attribute] { color: red; }
 testB[|attribute="fail"] { color: red; }
 testC[|attribute~="fail"] { color: red; }
 testD[|attribute^="fail"] { color: red; }
 testE[|attribute*="fail"] { color: red; }
 testF[|attribute$="fail"] { color: red; }
 testG[|attribute|="fail"] { color: red; }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
