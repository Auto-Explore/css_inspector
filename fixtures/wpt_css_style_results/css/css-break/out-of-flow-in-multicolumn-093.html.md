# css/css-break/out-of-flow-in-multicolumn-093.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-093.html"
}
```

## style[0]

```css

  ul { columns:2; column-gap:0; height:20px; width:200px; line-height:20px; margin-left:-100px; }
  ul li { position:relative; list-style-type:none; font-size:20px; }
  ul li::before { content:" \2022 "; position:absolute; top:-2px; left:-15px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
