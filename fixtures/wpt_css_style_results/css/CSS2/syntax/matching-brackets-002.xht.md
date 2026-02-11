# css/CSS2/syntax/matching-brackets-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/matching-brackets-002.xht"
}
```

## style[0]

```css
<![CDATA[
    p {
      background: red;
      color: green;
      color: red ] ) test-token \
       ~ ` ! @ # $ % ^ & * - _ + = | : > < ? / , .
       [\]\5D ']' "]"; background: red; } p { color: red; } ]
       (\)\29 ')' ")"; background: red; } p { color: red; } )
       '\'; background: red; } p { color: red; }',
       "\"; background: red; } p { color: red; }' p { color: red; } "
      ;
      background: white;
    }
  ]]>
```

```json
{
  "errors": 9,
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
      "message": "Invalid input.",
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
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
