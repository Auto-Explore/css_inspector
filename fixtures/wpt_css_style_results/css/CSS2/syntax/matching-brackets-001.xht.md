# css/CSS2/syntax/matching-brackets-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/matching-brackets-001.xht"
}
```

## style[0]

```css
<![CDATA[
    p {
      color: red;
      background: red;
    }
    #semicolon { background: transparent; }
    @foo ] } ) test-token \
     ~ ` ! @ # $ % ^ & * - _ + = | : > < ? / , .
     [\]\5D ']' "]"; # { background: red; } ]
     (\)\29 ')' ")"; #semicolon { background: red; } } } } )
     '; #semicolon { background: red; } } } }',
     "; #semicolon { background: red; }' } } }"
    ;
    #semicolon { color: green; }
    #block { background: transparent; }
    @foo ] } ) test-token \
     ~ ` ! @ # $ % ^ & * - _ + = | : > < ? / , .
     [\]\5D ']' "]"; #block { background: red; } ]
     (\)\29 ')' ")"; #block { background: red; } )
     '\'; #block { background: red; }',
     "\"; #block { background: red; }'"
     {\}\79 '}' "}"; #block { background: red; }
        #block { background: red; } }
    #block { color: green; }
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
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
