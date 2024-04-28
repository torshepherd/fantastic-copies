<script lang="ts">
	import { Presentation, Slide, Code, Media, FitText } from '@components'
	import { signal } from '@motion'
	import SlideTitle from './slideTitle.svelte'
	import BoxedText from './boxedText.svelte'
	import Demo1 from '@lib/fantastic_copies/demo1.svelte'
	import Demo2 from '@lib/fantastic_copies/demo2.svelte'
	import Demo3 from '@lib/fantastic_copies/demo3.svelte'
	import Demo4 from '@lib/fantastic_copies/demo4.svelte'

	const circle = signal(
		{ x: 0, y: 0, r: 80, fill: '#00ffff' },
		{ duration: 2000 }
	)

	const stackPointer = signal({ x: 0, textOpacity: 0 }, { duration: 500 })
</script>

<Presentation>
	<Slide image="static/fantastic-copies.png" animate>
		<!-- <Media
      class="h-[600px] w-full"
      src="https://www.youtube.com/embed/dQw4w9WgXcQ"
      type="iframe"
    /> -->
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Stack variables in C</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="c" class="basis-1/2 mt-[100px]">
				{`
        void foo() {
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Stack variables in C</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="c" class="basis-1/2 mt-[100px]">
				{`
        void foo() {
          int i = 2;
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Stack variables in C</SlideTitle>
		<Demo1 />
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Scalar copies</SlideTitle>
		<Demo2 />
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Struct (trivial) copies</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="c" lines="1-10|8|9" class="grow-0 mt-[20px]">
				{`
		struct S {
		  int i;
		  double j;
		  char k;
		};

		void foo() {
		  struct S object = {2, 3.14, 'n'};
		  struct S other = object; // ?
		  return;
		}
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Struct (trivial) copies</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="c" lines="9|10|11|12|13" class="grow-0 mt-[20px]">
				{`
        struct S {
          int i;
          double j;
          char k;
        };

        void foo() {
          struct S object = {2, 3.14, 'n'};
          struct S other;
          other.i = object.i;
          other.j = object.j;
          other.k = object.k;
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Vector in C</SlideTitle>
		<Demo3 />
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Vector in C</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="c" class="grow-0 mt-[50px]">
				{`
        struct vector {
          int* data;
          size_t size;
          size_t capacity;
        };

        void foo() {
          struct vector v = {NULL, 0U, 0U};
          vector_push_back(&v, 33);
          // Memory leak if we don't free!
          free(v.data);
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Vector in C++</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[100px]">
				{`
        void foo() {
          vector<int> v{}; // ?
          v.push_back(33);
          // ?
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>"Construction"</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]">
				{`
        void foo() {
          // allocate the stack memory
          char alloc[sizeof(vector<int>)];
          new (&alloc) vector<int>();
          auto v = (vector<int>*) &alloc;
          v.push_back(33);
          // ?
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>"Destruction"</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]">
				{`
        void foo() {
          // allocate the stack memory
          char alloc[sizeof(vector<int>)];
          new (&alloc) vector<int>();
          auto v = (vector<int>*) &alloc;
          v.push_back(33);
          v.~vector<int>();
          // release the stack memory
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Vector in C++</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[100px]">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Wait...</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[100px]">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          vector<int> other = v;
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Can we just trivial copy?</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]" lines="1-9|2|5|6">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          // pseudocode
          vector<int> other;
          other.data = v.data
          // ...
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Can we just trivial copy?</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]" lines="8-10">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          // pseudocode
          vector<int> other;
          other.data = v.data
          // ...
          // other.~vector<int>() frees other.data
          // v.~vector<int>() frees v.data again!
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Deep copies</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          // vector<int> other = v; expands to:
          char alloc[sizeof(vector<int>)];
          new (&alloc) vector<int>(v);
          auto other = (vector<int>*) &alloc;
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle>Deep copies</SlideTitle>
		<div class="grow flex gap-4">
			<Code lang="cpp" class="grow-0 mt-[50px]">
				{`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          // vector<int> other = v; expands to:
          char alloc[sizeof(vector<int>)];
          // (just like in C, the source is
          // supposed to be unchanged!)
          new (&alloc) vector<int>(v);
          auto other = (vector<int>*) &alloc;
          return;
        }
        `}
			</Code>
		</div>
	</Slide>

	<Slide animate>Is this always what we want?</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<SlideTitle
			>Vector would copy on resize<img
				data-src="static/notlikethis.png"
				class="h-12"
				alt=""
			/></SlideTitle
		>
		<Demo4 />
	</Slide>

	<!-- Slide with rewind effect on slideshow back to the "trivial copy case in C" then say "this is actually wrong" -->

	<!-- <Slide animate class="h-[600px] flex flex-col">
		<p class="font-bold text-8xl">🪄 Animotion</p>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<p class="font-bold text-6xl mt-8">🪄 Animotion</p>

		<div class="w-max mx-auto mt-16">
			<Code lang="ts" lines="1,4|2|3|1-4">
				{`
          const circle = signal(
            { x: 0, y: 200, r: 80, fill: '#00ffff' },
            { duration: 2000 }
          )
        `}
			</Code>
		</div>
	</Slide>

	<Slide on:out={() => circle.reset()} animate>
		<p class="font-bold text-6xl mt-8">🪄 Animotion</p>

		<div class="w-max mx-auto mt-16">
			<Code
				lang="ts"
				lines="1,4|2|3|1-4"
				steps={[
					['2', async () => await circle.to({ x: 400, fill: '#ffff00' })],
					['3', async () => await circle.to({ x: 0, fill: '#00ffff' })],
					['1-4', () => circle.reset()],
				]}
			>
				{`
          async function animate() {
            await circle.to({ x: 400, fill: '#ffff00' })
            await circle.to({ x: 0, fill: '#00ffff' })
          }
        `}
			</Code>
		</div>

		<svg
			class="w-full h-[400px] mx-auto"
			viewBox="0 0 400 400"
			data-id="animation"
		>
			<circle cx={$circle.x} cy={$circle.y} r={$circle.r} fill={$circle.fill} />
			<text
				x={$circle.x}
				y={$circle.y}
				font-size={$circle.r * 0.4}
				font-family="JetBrains Mono"
				text-anchor="middle"
				dominant-baseline="middle"
			>
				{$circle.x.toFixed(0)}
			</text>
		</svg>
	</Slide>

	<Slide animate class="h-[600px] flex flex-col">
		<p class="font-bold text-6xl mt-8">🪄 Animotion</p>
		<p class="mt-16 text-3xl">
			Learn more by reading the
			<a
				class="underline"
				href="https://animotion.pages.dev/docs"
				target="_blank"
			>
				Animotion docs
			</a>.
		</p>
	</Slide> -->
</Presentation>
