# css/CSS2/syntax/matching-brackets-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/matching-brackets-003.xht"
}
```

## style[0]

```css
<![CDATA[
  p { color: green; }
  p ( { border: solid red; } p { background: red; } )
  p { color: red; }
 ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css
<![CDATA[
    div {
      background: red;
      color: green;
    }
    ] ) test-token \
       ~ ` ! @ # $ % ^ & * - _ + = | : > < ? / , .
       [\]\5D ']' "]"; background: red; } div { color: red; } ]
       (\)\29 ')' ")"; background: red; } div { color: red; } )
       '\'; background: red; } div { color: red; }',
       "\"; background: red; } div { color: red; }' div { color: red; } "
    div { color: red; }
    div { background: white; }
  ]]>
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
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
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
