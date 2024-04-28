<script lang="ts">
	export let width: string
	export let height: string = width
	export let x: string
	export let y: string
	export let xOffset: string = '0'
	export let yOffset: string = '0'
	export let fontSize: string = '18'
	export let textFill: string = 'white'
	export let textOpacity: string

	export let fillOpacity: string = '1'
	export let strokeOpacity: string | null = null
	export let rectFill: string | null = null

	export let text: string = ''
</script>

<rect
	{width}
	{height}
	{x}
	{y}
	fill-opacity={fillOpacity}
	fill={rectFill}
	stroke-opacity={strokeOpacity}
/>
<text
	x={Number(x) + Number(width) / 2 + Number(xOffset)}
	y={Number(y) + Number(height) / 2 + Number(yOffset)}
	font-size={fontSize}
	text-anchor="middle"
	dominant-baseline="middle"
	stroke="none"
	opacity={textOpacity}
	fill={textFill}
>
	{text}
</text>
