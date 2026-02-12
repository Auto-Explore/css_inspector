# css/css-text-decor/text-emphasis-style-filled-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-emphasis-style-filled-001.xht"
}
```

## style[0]

```css
<![CDATA[
				.parent
				{
					border: solid 1px gray;
					font: 1.5em/2 monospace;
					width: 10em;
				}
				#test1
				{
					text-emphasis-style: filled;  /* missing shape computes to 'circle' */
				}
				#test2
				{
					text-emphasis-style: filled circle;
				}
				#test3
				{
					text-emphasis-style: filled dot;
				}
				#test4
				{
					text-emphasis-style: filled double-circle;
				}
				#test5
				{
					text-emphasis-style: filled sesame;
				}
				#test6
				{
					text-emphasis-style: filled triangle;
				}
		]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
