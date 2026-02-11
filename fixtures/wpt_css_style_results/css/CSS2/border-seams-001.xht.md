# css/CSS2/border-seams-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/border-seams-001.xht"
}
```

## style[0]

```css
<![CDATA[
   #outer { width: 250px;
            border: 50px solid;
            border-color: #FEFEFE #FDFDFD #FEFEFE #FEFEFE;
            background: red;
            }
   #inner { background: white;
            border: 20px solid;
            border-color: #FCFCFC; #FCFCFC; #FDFDFD #FCFCFC;
            }
  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
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
