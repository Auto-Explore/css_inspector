# css/css-conditional/at-supports-048.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-048.html"
}
```

## style[0]

```css

			div {
				background:red;
				height:100px;
				width:100px;
				margin: 10px;
				border: solid 1px black;
			}
			.test-1 {
			    background-color: red;
			    @supports (background-color: red) {
			        background-color: green;
			    }
			}
			.test-2 {
			    background-color: green;
			    @supports (invalid-declaration: foobar) {
			        background-color: red;
			    }
			}
			.test-3 {
			    background-color: green;
			    @supports (color: red) {
			        color: red;
			    }
			}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
