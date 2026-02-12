# css/css-text/text-indent/text-indent-percentage-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-text/text-indent/text-indent-percentage-001.xht"
}
```

## style[0]

```css
<![CDATA[
			#parent
			{
				font: 16px/1em Ahem;
				position: relative;
				width: 400px;
			}
			#reference1
			{
				color: red;
				left: 100px; /* see comments for #test1 below */
				position: absolute;
				top: 0;
				z-index: -1;
			}
			#reference2
			{
				margin-left: 50%;
			}
			#test1
			{
    			margin-left: -50%; /* -50% * 400px = -200px which makes the inline-size of this block 600px */
    			text-indent: 50%;  /* 50% * 600px = 300px (which is 100px from the start of #parent due to the negative margin) */
			}
			#test2
			{
    			text-indent: 50%;
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
