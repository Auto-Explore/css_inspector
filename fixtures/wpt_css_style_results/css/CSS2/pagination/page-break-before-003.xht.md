# css/CSS2/pagination/page-break-before-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-before-003.xht"
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
		@top-center {
           content: "[This page intentionally left blank]"
        }
	}
	@page :first {
		@top-center {
           content: ""
        }
		border: 1em solid blue;
		margin: 7%;
	}
	p.spacer {page-break-after: always;}
	p.goLeft {
		page-break-before: left;
	}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
