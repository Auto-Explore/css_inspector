# css/CSS2/pagination/page-break-before-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-before-005.xht"
}
```

## style[0]

```css

	@page {margin: 7%;}
	@page :left {
		margin-right: 50%;
	}
	@page :right {
		margin-left: 50%;
	}
	@page :first {
		@top-center {
           content: ""
        }
		border: 1em solid blue;
		margin: 7%;
	}
	p.spacer {page-break-after: always;}
	p.goRight {
		page-break-before: right;
	}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
