# css/css-color/t421-rgb-func-whitespace-b.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t421-rgb-func-whitespace-b.xht"
}
```

## style[0]

```css
<![CDATA[
		html, body { background: black; }
		#one { color: rgb(  0,
255		,0); }
		#two { color: rgb(0
,255
,0); }
		#three { color: rgb(     0   ,   255    	  , 			0 ); }

		#four { color: rgb(  0%,
100%		,0%); }
		#five { color: rgb(0%
,100%
,0%); }
		#six { color: rgb(     0%   ,   100%    	  , 			0% ); }
		]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
