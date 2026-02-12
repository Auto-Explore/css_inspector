# css/CSS2/syntax/signed-numbers-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/signed-numbers-002.xht"
}
```

## style[0]

```css

  div {background: red; width: 20px; height: 100px}
  div div {background: green; height: 20px}
  .s1 {margin-left: + 30px}			/* Invalid */
  .s2 {margin-left: - 10px}			/* Invalid */
  .s3 {margin-left: -/**/10px}			/* Invalid */
  .s4 {margin-left: 30px; margin-left: +00px}	/* Valid */
  .s5 {margin-left: 30px; margin-left: -0.00px}	/* Valid */
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
