# css/CSS2/syntax/invalid-decl-at-rule-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/invalid-decl-at-rule-001.xht"
}
```

## style[0]

```css

            /*<![CDATA[*/
                @media screen
                {
                    div
                    {
                        color: green;
                    }
                    div&p
                    {
                        red: isbad;
                    }
                }
            /*]]>*/
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
