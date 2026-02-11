# css/CSS2/syntax/comments-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/comments-003.xht"
}
```

## style[0]

```css

            <![CDATA[
            div
            {
              color: green;
            }
            <!--
            /*
            #div1
            {
                color: red;
            }
            -->
            #div1
            {
                color: red;
            }
            ]]>
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unclosed comment.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
