# css/CSS2/syntax/invalid-decl-at-rule-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/invalid-decl-at-rule-002.xht"
}
```

## style[0]

```css

            /*<![CDATA[*/
                @media screen
                {
                    div&p
                    {
                        red: isbad;
                    }
                    div
                    {
                        color: green;
                    }
                }
            /*]]>*/
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “red”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
