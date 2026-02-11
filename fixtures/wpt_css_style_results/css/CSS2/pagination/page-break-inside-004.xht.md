# css/CSS2/pagination/page-break-inside-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-inside-004.xht"
}
```

## style[0]

```css

	* {
		margin: 0;
		padding: 0;
		orphans: 1;
		widows: 1;
	}
	html, body {
		height:100%;
		line-height: 1;
		font-size: 20px;
	}
	div.spacers {
		height: 50%;
	}
	div#takeTwo {page-break-before: always}
	div.backUp {margin-top: -4em;}
	div#break2 {
		page-break-inside: avoid;
		page-break-inside: auto;
	}
	.breaker {
		width: 0;
		font-weight: bold;
		color: blue;
	}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
