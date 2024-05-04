<script lang="ts">
  import {
    Presentation,
    Slide,
    Code,
    Media,
    FitText,
    Step,
    Notes,
  } from '@components'
  import { animate, signal } from '@motion'
  import SlideTitle from './slideTitle.svelte'
  import BoxedText from './boxedText.svelte'
  import Demo1 from '@lib/fantastic_copies/demo1.svelte'
  import Demo2 from '@lib/fantastic_copies/demo2.svelte'
  import Demo3 from '@lib/fantastic_copies/demo3.svelte'
  import Demo4 from '@lib/fantastic_copies/demo4.svelte'
  import Tree from '@lib/fantastic_copies/tree3.svelte'
  import Tree1 from '@lib/fantastic_copies/tree1.svelte'
  import Tree2 from '@lib/fantastic_copies/tree2.svelte'

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
      preload={true}
    /> -->
  </Slide>

  <!-- <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Stack variables in C</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" class="basis-1/2 mt-[100px]">
        {`
        void foo() {
          return;
        }
        `}
      </Code>
    </div>
  </Slide> -->

  <!-- <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Stack variables in C</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" class="basis-1/2 mt-[100px]">
        {`
        void foo() {
          int i = 2;
          return;
        }
        `}
      </Code>
    </div>
  </Slide> -->

  <Slide animate class="h-[600px] flex flex-col justify-center">
    <div class="grow flex gap-4 justify-center">
      <div class="flex flex-col justify-center">
        <img
          class="h-[250px] w-full object-cover rounded-3xl"
          alt=""
          data-src="static/IMG_4084.jpg"
        />
        <p class="mt-[20px] text-2xl">Tor Shepherd</p>
        <p class="mt-[20px] text-xl">Motion Planning</p>
        <p class="mt-[-8px] text-xl flex gap-1 justify-center">
          @ Motional <img
            data-src="static/motional.png"
            alt=""
            class="h-[14px] mt-2"
          />
        </p>
      </div>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col justify-center">
    <div class="grow flex gap-4 justify-center">
      <div class="flex flex-col justify-center">
        <img
          class="h-[250px] w-full object-cover rounded-3xl"
          alt=""
          data-src="static/IMG_4084.jpg"
        />
        <p class="mt-[20px] text-2xl">Tor Shepherd</p>
        <p class="mt-[20px] text-xl">Motion Planning</p>
        <p class="mt-[-8px] text-xl flex gap-1 justify-center">
          @ Motional <img
            data-src="static/motional.png"
            alt=""
            class="h-[14px] mt-2"
          />
        </p>
      </div>
      <div class="flex flex-col justify-center">
        <Code lang="cpp" class="grow ml-[80px] mt-[200px]">
          {`
          // Why do we use C++ ?
          `}
        </Code>
      </div>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col justify-center">
    <div class="grow flex gap-4 justify-center">
      <div class="flex flex-col justify-center">
        <img
          class="h-[250px] w-full object-cover rounded-3xl"
          alt=""
          data-src="static/IMG_4084.jpg"
        />
        <p class="mt-[20px] text-2xl">Tor Shepherd</p>
        <p class="mt-[20px] text-xl">Motion Planning</p>
        <p class="mt-[-8px] text-xl flex gap-1 justify-center">
          @ Motional <img
            data-src="static/motional.png"
            alt=""
            class="h-[14px] mt-2"
          />
        </p>
      </div>
      <div class="flex flex-col justify-center">
        <Code lang="cpp" class="grow ml-[80px] mt-[200px]">
          {`
          // Why do we use C++ ?

          // Convenient?
          // Fast?
          `}
        </Code>
      </div>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col justify-center">
    <div class="grow flex gap-4 justify-center">
      <div class="flex flex-col justify-center">
        <img
          class="h-[250px] w-full object-cover rounded-3xl"
          alt=""
          data-src="static/IMG_4084.jpg"
        />
        <p class="mt-[20px] text-2xl">Tor Shepherd</p>
        <p class="mt-[20px] text-xl">Motion Planning</p>
        <p class="mt-[-8px] text-xl flex gap-1 justify-center">
          @ Motional <img
            data-src="static/motional.png"
            alt=""
            class="h-[14px] mt-2"
          />
        </p>
      </div>
      <div class="flex flex-col justify-center">
        <Code lang="cpp" class="grow ml-[80px] mt-[200px]">
          {`
          // Why do we use C++ ?

          // Convenient to write
          // Fast code
          `}
        </Code>
      </div>
    </div>
    <Notes>
      <br /> This talk is ostensibly about "copies in C++", but really it's about
      this promise that C++ abstracts away the details of patterns we like in C,
      but that are inconvenient to write in C
    </Notes>
  </Slide>

  <Slide animate class="flex flex-col justify-center">
    <div class="grow flex gap-4 justify-center">
      <div class="basis-1/3 text-3xl">
        Part I
        <br />
        <br />
        Basic types of copies
      </div>
      <Step class="basis-1/3 text-3xl" fadeLeft>
        Part II
        <br />
        <br />
        Everything from Part I was wrong
      </Step>
      <Step class="basis-1/3 text-3xl" fadeLeft>
        Part III
        <br />
        <br />
        What can we do about it
      </Step>
    </div>
  </Slide>

  <Slide animate>Let's write some C 😎</Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Scalar copies</SlideTitle>
    <Demo2 />
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Struct (trivial) copies</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" lines="1-10|9" class="grow-0 mt-[20px]">
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
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" lines="9-12" class="grow-0 mt-[20px]">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
    <div class="grow flex gap-4 justify-center">
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
      >Vector would copy on resize <img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <Demo4 />
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        class vector {
          // Copy constructor
          vector(const vector& other) {
            this->data = malloc(other.size * ...);
            for (...) { this->data[i] = other.data[i]; }
          }
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        class vector {
          // Copy constructor
          vector(const vector& other) {
            this->data = malloc(other.size * ...);
            for (...) { this->data[i] = other.data[i]; }
          }
          // Move constructor
          vector(/* ?? */ other) {
            this->data = other.data;
          }
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        class vector {
          // Copy constructor
          vector(const vector& other) {
            this->data = malloc(other.size * ...);
            for (...) { this->data[i] = other.data[i]; }
          }
          // Move constructor
          vector(/* ?? */ other) {
            this->data = other.data;
            other.data = nullptr;
          }
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        class vector {
          // Copy constructor
          vector(const vector& other) {
            this->data = malloc(other.size * ...);
            for (...) { this->data[i] = other.data[i]; }
          }
          // Move constructor
          vector(vector&& other) {
            this->data = other.data;
            other.data = nullptr;
          }
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          vector<int> copied = v;
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          vector<int> copied = v;
          vector<int> moved = 
            static_cast<vector<int>&&>(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>"Move semantics"</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          vector<int> copied = v;
          vector<int> moved = std::move(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Family tree of copies</SlideTitle>
    <Tree1 />
  </Slide>

  <Slide animate class="flex flex-col">
    <FitText class="">Part II: So that's it?</FitText>
    <div class="flex gap-4 justify-center">
      <Step fadeUp>
        <img
          class="mt-[50px] h-[200px] mx-auto"
          data-src="static/yes-but-no.jpg"
          alt="Well yes, but actually no"
        />
      </Step>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[100px]">
        {`
        struct S {
          std::vector<int> data;
        };
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[100px]">
        {`
        struct S {
          S(const S&) = default;
          S(S&&) = default;
          std::vector<int> data;
        };
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[20px]" lines="1-14|3">
        {`
        struct S {
          S(const S&) = default;
          S(S&&) = default;
          std::vector<int> data;
        };

        void foo() {
          vector<S> v{};
          v.emplace_back();
          v.back().data.push_back(33);
          v.emplace_back();
          // moves v[0] over with S(S&&)
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[20px]" lines="3|11-12">
        {`
        struct S {
          S(const S&) = default;
          S(S&&);
          std::vector<int> data;
        };

        void foo() {
          vector<S> v{};
          v.emplace_back();
          v.back().data.push_back(33);
          v.emplace_back();
          // ???
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[20px]" lines="11-12">
        {`
        struct S {
          S(const S&) = default;
          S(S&&);
          std::vector<int> data;
        };

        void foo() {
          vector<S> v{};
          v.emplace_back();
          v.back().data.push_back(33);
          v.emplace_back();
          // Deep copies??? Why???
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 Exceptions</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[20px]">
        {`
        struct S {
          S(const S&) noexcept = default;
          S(S&&) noexcept(false);
          // Assumes this may throw an exception...
          std::vector<int> data;
        };
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Exception guarantees</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[500px] w-[800px]"
        src="https://en.cppreference.com/w/cpp/container/vector/emplace_back"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <!-- <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Exception guarantees</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <ul class="grow-0 mt-[30px] text-left text-3xl">
        <li class="list-disc">
          if move ctor is noexcept -> <strong class="text-green-400"
            >move :)</strong
          >
        </li>
        <Step fadeUp
          ><li class="list-disc">
            else if copyable -> <strong class="text-red-400">copy :(</strong>
          </li></Step
        >
        <Step fadeUp
          ><li class="list-disc">
            else, or with -fno-exceptions -> <strong class="text-green-400"
              >move :)</strong
            >
          </li></Step
        >
      </ul>
    </div>
  </Slide> -->

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>std::move_if_noexcept</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[500px] w-[800px]"
        src="https://en.cppreference.com/w/cpp/utility/move_if_noexcept"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <Slide animate>Move Semantics 💔 const</Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 const</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<int> v{};
          v.push_back(33);
          vector<int> copied = v;
          vector<int> moved = std::move(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 const</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          const vector<int> v{33};
          vector<int> copied = v;
          vector<int> moved = std::move(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 const</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          const vector<int> v{33};
          vector<int> copied = v;
          vector<int> moved = std::move(v);
          // Silently copies :(
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate>🌊Ripple effects in the standard library🌊</Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      >shared_future::get() copies<img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          shared_future<string> resultFuture
            = threadPool.submit(doStuff);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      >shared_future::get() copies<img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          shared_future<string> resultFuture
            = threadPool.submit(doStuff);
          string result
            = std::move(resultFuture.get());
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      >shared_future::get() copies<img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          shared_future<string> resultFuture
            = threadPool.submit(doStuff);
          string result
            = std::move(resultFuture.get());
          // nice try 🤣 *deep copies your string*
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      >shared_future::get() copies<img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[500px] w-[800px]"
        src="https://en.cppreference.com/w/cpp/thread/shared_future/get"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Surely this is fine...?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[100px]">
        {`
        void foo() {
          vector<vector<int>> v{{1, 2, 3}, {4, 5, 6}};
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Surely this is fine...?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<vector<int>> v{
            initializer_list<vector<int>>{
              vector<int>{1, 2, 3}, vector<int>{4, 5, 6}
            }
          };
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>What the heck is an init list?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[378px] w-[800px]"
        src="https://en.cppreference.com/w/cpp/utility/initializer_list"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>So this...</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<vector<int>> v{{1, 2, 3}, {4, 5, 6}};
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>... is equivalent to this</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          const vector<int> a{1, 2, 3};
          const vector<int> b{4, 5, 6};
          vector<vector<int>> v;
          v.push_back(a);
          v.push_back(b);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      >vector(initializer_list) copies<img
        data-src="static/notlikethis.png"
        class="h-10"
        alt=""
      /></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          const vector<int> a{1, 2, 3};
          const vector<int> b{4, 5, 6};
          vector<vector<int>> v;
          v.push_back(a);
          // 🤣 *deep copies your vector*
          v.push_back(b);
          // 🤣 *deep copies your vector*
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Basic vector usage prevents move??</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[100px]">
        {`
        void foo() {
          vector<vector<int>> v{{1, 2, 3}, {4, 5, 6}};
          // 🤣 *deep copies your vectors*
          return;
        }
        `}
      </Code>
      <img
        data-src="static/disbelief.gif"
        class="h-[160px] mt-[85px] mr-[80px]"
        alt=""
      />
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Why can't we move from const?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <ul class="grow-0 mt-[50px] text-center space-y-8">
        <Step fadeUp><li><q>performance issue, not a bug</q></li></Step>
        <Step fadeUp><li><q>generic code can fall back to copy</q></li></Step>
        <Step fadeUp>
          <li>
            The real reason:<br /><i class="text-cyan-400">
              move semantics are second-class in C++
            </i>
          </li>
        </Step>
      </ul>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[100px]">
        {`
        void foo() {
          vector<int> v{1, 2, 3};
          const auto other = std::move(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        void foo() {
          vector<int> v{1, 2, 3};
          // v.data = malloc(...);
          const auto other = std::move(v);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[10px]">
        {`
        void foo() {
          vector<int> v{1, 2, 3};
          // v.data = malloc(...);
          const auto other = std::move(v);
          // other.data = v.data;
          // v.data = nullptr;
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[10px]">
        {`
          void foo() {
            vector<int> v{1, 2, 3};
            // v.data = malloc(...);
            const auto other = std::move(v);
            // other.data = v.data;
            // v.data = nullptr;
            // destroy other:
            //   free(other.data);
            return;
          }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[10px]">
        {`
        void foo() {
          vector<int> v{1, 2, 3};
          // v.data = malloc(...);
          const auto other = std::move(v);
          // other.data = v.data;
          // v.data = nullptr;
          // destroy other:
          //   free(other.data);
          // destroy v:
          //   free(nullptr); // noop
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code
        lang="c"
        class="grow-0 mt-[30px] text-sm mx-[20px]"
        id="C-comparison"
      >
        {`
        // C
        void foo() {
          vector_t v
            = {malloc(...), 3};
          v.data[0] = 1; ...
          const vector_t other = v;
          // other.data = v.data;
          free(other.data);
          return;
        }
        `}
      </Code>
      <Code lang="cpp" class="grow-0 mt-[30px] text-sm mx-[20px]">
        {`
        // C++
        void foo() {
          vector<int> v{1, 2, 3};
          // v.data = malloc(...);
          const auto other = std::move(v);
          // other.data = v.data;
          // v.data = nullptr;
          // destroy other:
          //   free(other.data);
          // destroy v:
          //   free(nullptr); // noop
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Are move semantics what we actually meant?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" class="grow-0 mt-[30px] text-sm" id="C-comparison">
        {`
        // C
        void foo() {
          vector_t v
            = {malloc(...), 3};
          v.data[0] = 1; ...
          const vector_t other = v;
          // other.data = v.data;
          free(other.data);
          return;
        }
        `}
      </Code>
      <Code lang="cpp" class="grow-0 mt-[30px] text-sm">
        {`
        // C++
        void foo() {
          vector<int> v{1, 2, 3};
          // v.data = malloc(...);
          const auto other = std::move(v);
          // other.data = v.data;
          // v.data = nullptr;
          // destroy other:
          //   free(other.data);
          // destroy v:
          //   free(nullptr); // noop
          return;
        }
        `}
      </Code>
      <Code lang="rs" class="grow-0 mt-[30px] text-sm" id="C-comparison">
        {`
        // Rust
        fn foo() {
          let mut v = vec![1, 2, 3];
          // calls malloc(...);
          let other = v;
          // trivially copies v to other
          // calls core::mem::forget(v);
          // destroy other:
          //   free(...)
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Wait, you can totally move from const?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="rs" class="grow-0 mt-[30px]" id="Rust-comparison">
        {`
        // Rust
        fn foo() {
          let mut v = vec![1, 2, 3];
          // calls malloc(...);
          let other = v;
          // trivially copies v to other
          // calls core::mem::forget(v);
          // destroy other:
          //   free(...)
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Wait, you can totally move from const?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="rs" class="grow-0 mt-[30px]" id="Rust-comparison">
        {`
        // Rust
        fn foo() {
          let v = vec![1, 2, 3];
          // calls malloc(...);
          let other = v;
          // trivially copies v to other
          // calls core::mem::forget(v);
          // destroy other:
          //   free(...)
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <div class="grow flex gap-4 justify-center text-2xl">
      <table class="table-fixed border-none">
        <thead>
          <tr>
            <th scope="col" class="font-normal border-none">std::move</th>
            <th scope="col" class="font-normal border-none"
              >"trivial-relocation"</th
            >
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="col" class="font-normal border-none"
              >-> mutate the original object at runtime</th
            >
            <th scope="col" class="font-normal border-none"
              >-> mutate the original <strong>type</strong> at compile-time</th
            >
          </tr>
          <tr>
            <th scope="col" class="font-normal border-none"
              >-> a trivial copy <strong>and</strong> preventing object from destroying
              resources</th
            >
            <th scope="col" class="font-normal border-none"
              >-> a single trivial copy</th
            >
          </tr>
        </tbody>
      </table>
    </div>
  </Slide>

  <Slide animate>What's so great about trivial copies anyway?</Slide>

  <!-- <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Revisiting trivial copies</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[378px] w-[800px]"
        src="http://localhost:5173/#/5/0/1"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide> -->

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Flashback: trivial copies</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" lines="9-12" class="grow-0 mt-[20px]">
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
    <SlideTitle>What do trivial copies compile to?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-full w-full"
        src="https://godbolt.org/z/osYocef8o"
        type="iframe"
        preload={true}
      />
      <Notes>
        <br /> Look at both implementations of copy
        <br /> See how the first one only has a single block of asm, whereas the
        second has multiple?
        <br /> Kind of hard to see what's happening though. So if you click on plus,
        you can open "LLVM IR"
      </Notes>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle
      ><a
        class="underline text-[var(--r-link-color)]"
        href="https://raw.githubusercontent.com/gcc-mirror/gcc/master/libgcc/memcpy.c"
        >gcc/libgcc/memcpy.c</a
      ></SlideTitle
    >
    <div class="grow flex gap-4 justify-center">
      <Code lang="c" class="grow-0 mt-[50px]">
        {`
        /* Public domain.  */
        #include <stddef.h>

        void *
        memcpy (void *dest, const void *src, size_t len)
        {
          char *d = dest;
          const char *s = src;
          while (len--)
            *d++ = *s++;
          return dest;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>memcpy-coalescing</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-full w-full"
        src="https://godbolt.org/z/rj9eMh35c"
        type="iframe"
        preload={true}
      />
      <Notes>
        <br /> Reminder to remove -mno-sse
      </Notes>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>memcpy-coalescing for arrays</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        struct S {
          int i{};
          double j{};
          char k{};
        };

        void foo() {
          vector<S> v;
          v.resize(4);
          v.resize(8);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>memcpy-coalescing for arrays</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        struct S {
          int i{};
          double j{};
          char k{};
        };

        void foo() {
          vector<S> v;
          v.resize(4);
          // v.data = malloc(4 * 24)
          v.resize(8);
          // v.new_data = malloc(8 * 24)
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>memcpy-coalescing for arrays</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[20px]">
        {`
        struct S {
          int i{};
          double j{};
          char k{};
        };

        void foo() {
          vector<S> v;
          v.resize(4);
          // v.data = malloc(4 * 24)
          v.resize(8);
          // v.new_data = malloc(8 * 24)
          // memcpy(v.new_data, v.data, 4 * sizeof(S))
          // v.data = v.new_data
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate>We ❤️ bulk-<code>memcpy</code></Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>What about movable types?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        void foo() {
          vector<vector<int>> v;
          v.resize(4);
          v.resize(8);
          // For each element:
          // - std::move()
          // - ~vector<int>(nullptr)
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Trivial relocation</SlideTitle>
    <div class="grow-0 flex gap-4 justify-center h-[300px]">
      <Code lang="cpp" class="grow-0 mt-[50px]">
        {`
        void foo() {
          vector<vector<int>> v;
          v.resize(4);
          v.resize(8);
          // memcpy(v.new_data, v.data, 4 * sizeof(vector<int>))
          // v.data = v.new_data
          // prevent original destructors
        }
        `}
      </Code>
    </div>
    <a
      class="underline text-[var(--r-link-color)] text-lg mt-8"
      href="https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/p1144r7.html"
    >
      std::is_trivially_relocatable proposal
    </a>
    <div class="grow">
      <a
        class="underline text-[var(--r-link-color)] text-lg"
        href="https://www.youtube.com/watch?v=SGdfPextuAU"
      >
        One of Arthur O'Dwyer's CppCon talks
      </a>
    </div>
  </Slide>

  <Slide animate>You actually didn't mean to move anyway</Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>What about function boundaries?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar();

        void foo() {
          vector<int> v = bar();
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Does it copy?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar();

        void foo() {
          vector<int> v = bar();
          // does it create vector<int>
          // then pass it into
          // vector<int>(const vector<int>&) ?
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>... or maybe it moves?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar();

        void foo() {
          vector<int> v = bar();
          // does it create vector<int>
          // then pass it into
          // vector<int>(vector<int>&&) ?
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle><q>Guaranteed copy elision</q></SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        void barImpl(char* returnSlot);

        void foo() {
          // pseudocode:
          char alloc[sizeof(vector<int>)];
          barImpl(alloc);
          auto v = (vector<int>*) &alloc;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>What happens inside the function?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar() {
          vector<int> temp{1, 2, 3};
          temp.push_back(4);
          return temp;
        }

        void barImpl(char* returnSlot);
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Return Value Optimization</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar() {
          vector<int> temp{1, 2, 3};
          temp.push_back(4);
          return temp;
        }

        void barImpl(char* returnSlot) {
          new (&returnSlot) vector<int>(1, 2, 3);
          ((vector<int>*)returnSlot)->push_back(4);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Not a copy in sight!</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar() {
          vector<int> temp{1, 2, 3};
          temp.push_back(4);
          return temp;
        }

        void foo() {
          vector<int> v = bar();
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate>RVO is extremely easy to break</Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 RVO</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar() {
          vector<int> temp{1, 2, 3};
          return std::move(temp);
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Move Semantics 💔 RVO</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar() {
          vector<int> temp{1, 2, 3};
          return std::move(temp);
          // Return type is vector<int>,
          // so we have to convert!
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle><q>Can we have multiple returns?</q></SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int>, vector<int> bar() {
          vector<int> temp1{1, 2, 3};
          vector<int> temp2{1, 2, 3};
          return temp1, temp2;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle><q>We have multiple returns at home:</q></SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        pair<vector<int>, vector<int>> bar() {
          vector<int> temp1{1, 2, 3};
          vector<int> temp2{1, 2, 3};
          return {temp1, temp2};
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>No RVO? Maybe auto-move</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        vector<int> bar(bool cond) {
          vector<int> temp1{1, 2, 3};
          vector<int> temp2{1, 2, 3};
          if (cond)
            return temp1;
          return temp2;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>No RVO? Maybe auto-move</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        void barImpl(bool cond, char* returnSlot) {
          vector<int> temp1{1, 2, 3};
          vector<int> temp2{1, 2, 3};
          if (cond) {
            new (&returnSlot) vector<int>(temp1);
            return;
          }
          new (&returnSlot) vector<int>(temp2);
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>No RVO? Maybe auto-move</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        void barImpl(bool cond, char* returnSlot) {
          vector<int> temp1{1, 2, 3};
          vector<int> temp2{1, 2, 3};
          if (cond) {
            new (&returnSlot) vector<int>(std::move(temp1));
            return;
          }
          new (&returnSlot) vector<int>(std::move(temp2));
          return;
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Family tree of copies</SlideTitle>
    <Tree />
  </Slide>

  <Slide animate>
    <FitText class="">Part III: Doing something about it</FitText>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>How do we track down copies?</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[150px]">
        {`
        class PleaseDontCopy {
          // ...
        };
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Delete the copy constructor</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        class PleaseDontCopy {
          PleaseDontCopy(const PleaseDontCopy&) = delete;
          // ...
        };

        void foo() {
          PleaseDontCopy a{};
          auto b = a; // error
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Deprecate the copy constructor</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        class PleaseDontCopy {
          [[deprecated]]
          PleaseDontCopy(const PleaseDontCopy&) = default;
          // ...
        };

        void foo() {
          PleaseDontCopy a{};
          auto b = a; // warning (-Wdeprecated-declarations)
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Make copies explicit</SlideTitle>
    <div class="grow-0 flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[60px]">
        {`
        class PleaseDontCopy {
          explicit
          PleaseDontCopy(const PleaseDontCopy&) = default;
          // ...
        };

        void foo() {
          PleaseDontCopy a{};
          auto b = PleaseDontCopy{a};
        }
        `}
      </Code>
    </div>
    <div class="grow mt-[50px]">
      <a
        class="underline text-[var(--r-link-color)] text-lg"
        href="https://www.youtube.com/watch?v=5wJ-jKK_Zy0"
      >
        C++ Weekly - Ep 241 - Using `explicit` to Find Expensive Accidental
        Copies
      </a>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Announce copies</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[10px]">
        {`
        class TraceMe {
          TraceMe(const TraceMe&) {
            std::println("Copied!");
            PERFETTO_TRACE("Copied!");
          }
        };

        class PleaseDontCopy : TraceMe {
          // ...
        };

        void foo() {
          PleaseDontCopy a{};
          auto b = a; // prints at runtime
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate
    >&lt;&lt;
    <Step fadeLeft>(shift left, get it?)</Step>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Compiler warnings</SlideTitle>
    <div class="grow-0 flex gap-4 justify-center">
      <img
        class="mt-[20px] h-[250px] mx-auto"
        data-src="static/knives.jpg"
        alt=""
      />
    </div>
    <div class="grow flex gap-4 justify-center">
      <ul class="grow-0 mt-[50px] text-left">
        <Step fadeUp><li class="list-disc text-2xl">-Wmove</li></Step>
        <Step fadeUp
          ><li class="list-disc text-2xl">-Wlarge-by-value-copy</li></Step
        >
      </ul>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>clang-tidy is awesome</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[500px] w-[800px]"
        src="https://clang.llvm.org/extra/clang-tidy/checks/list.html#clang-tidy-checks"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Missing check? No problem 😎</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Media
        class="h-[500px] w-[1600px]"
        src="https://clang.llvm.org/extra/clang-tidy/Contributing.html#getting-involved"
        type="iframe"
        preload={true}
      />
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Missing check? No problem 😎</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lines={null} lang="bash" class="grow-0 mt-[100px]">
        {`
        $ git clone https://github.com/llvm/llvm-project
        $ cd llvm/clang-tools-extra
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Missing check? No problem 😎</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lines={null} lang="bash" class="grow-0 mt-[100px]">
        {`
        $ git clone https://github.com/llvm/llvm-project
        $ cd llvm/clang-tools-extra
        $ clang-tidy/add_new_check.py performance vector-init-list
        # Creating clang-tidy/performance/VectorInitListCheck.cpp...
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Test in clang-query</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lang="cpp" class="grow-0 mt-[50px] text-sm">
        {`
        struct Test {
          std::string Data;
          int Data2;
        };
        
        std::vector<Test> TestFn(int i) {
          std::vector<std::string> V({"", "", {}});
          std::vector<std::string> V1{"", ""};

          return {{"hi", 1}, {}, {"there", 3}};
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Test in clang-query</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lines={null} lang="isbl" id="clang-query" class="grow-0 mt-[150px]">
        {`
        match cxxConstructExpr()
        `}
      </Code>
      <Code lang="cpp" class="grow-0 mt-[50px] text-sm">
        {`
        struct Test {
          std::string Data;
          int Data2;
        };

        std::vector<Test> TestFn(int i) {
          std::vector<std::string> V({"", "", {}});
          //                       ^  ^   ^   ^
          std::vector<std::string> V1{"", ""};
          //                       ^  ^   ^

          return {{"hi", 1}, {}, {"there", 3}};
          //     ^^^         ^   ^^
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Test in clang-query</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lines={null} lang="isbl" id="clang-query" class="grow-0 mt-[130px]">
        {`
        match cxxConstructExpr(
          has(cxxStdInitializerListExpr())
        )
        `}
      </Code>
      <Code lang="cpp" class="grow-0 mt-[50px] text-sm">
        {`
        struct Test {
          std::string Data;
          int Data2;
        };

        std::vector<Test> TestFn(int i) {
          std::vector<std::string> V({"", "", {}});
          //                       ^~~~~~~~~~~~~~~
          std::vector<std::string> V1{"", ""};
          //                       ^~~~~~~~~~

          return {{"hi", 1}, {}, {"there", 3}};
          //     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Test in clang-query</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code
        lines={null}
        lang="isbl"
        id="clang-query"
        class="grow-0 mt-[50px] text-sm"
      >
        {`
        match cxxConstructExpr(
          has(cxxStdInitializerListExpr()),
          hasDeclaration(
            cxxConstructorDecl(
              ofClass(
                cxxRecordDecl(
                  classTemplateSpecializationDecl(
                    hasName("::std::vector")
                  )
                )
              )
            )
          )
        )
        `}
      </Code>
      <Code lang="cpp" class="grow-0 mt-[50px] text-sm">
        {`
        struct Test {
          std::string Data;
          int Data2;
        };

        std::vector<Test> TestFn(int i) {
          std::vector<std::string> V({"", "", {}});
          //                       ^~~~~~~~~~~~~~~
          std::vector<std::string> V1{"", ""};
          //                       ^~~~~~~~~~

          return {{"hi", 1}, {}, {"there", 3}};
          //     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        }
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Compile your own clang-tidy</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <Code lines={null} lang="bash" class="grow-0 mt-[100px]">
        {`
        $ git clone https://github.com/llvm/llvm-project
        $ cd llvm/clang-tools-extra
        $ clang-tidy/add_new_check.py performance vector-init-list
        # Creating clang-tidy/performance/VectorInitListCheck.cpp...
        $ cmake ... -DLLVM_ENABLE_PROJECTS="clang-tools-extra"
        $ ninja install
        `}
      </Code>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>Takeaways</SlideTitle>
    <div class="grow flex gap-4 justify-center">
      <ul class="grow-0 mt-[50px] text-left">
        <Step fadeUp>
          <li class="list-disc text-2xl">
            Design the system to avoid dynamic ownership of data
          </li>
          <li class="text-2xl">
            (Do like llama.cpp does! <code>mmap</code> your data into the program
          </li>
          <li class="text-2xl">and leave it there!)</li>
        </Step>
        <Step fadeUp>
          <li class="list-disc text-2xl mt-[12px]">Use simple types</li>
        </Step>
        <Step fadeUp>
          <li class="list-disc text-2xl mt-[12px]">Trace your programs</li>
        </Step>
        <Step fadeUp>
          <li class="list-disc text-2xl mt-[12px]">
            Use compiler warnings and linters
          </li>
        </Step>
      </ul>
    </div>
  </Slide>

  <Slide animate class="h-[600px] flex flex-col">
    <SlideTitle>What are we missing?</SlideTitle>
    <div class="grow-0 flex gap-4 justify-center mt-[10px]">
      <svg
        width="300px"
        height="250px"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M7.5 4.5C7.5 3.11929 8.61929 2 10 2C11.3807 2 12.5 3.11929 12.5 4.5V6H13.5C14.8978 6 15.5967 6 16.1481 6.22836C16.8831 6.53284 17.4672 7.11687 17.7716 7.85195C18 8.40326 18 9.10218 18 10.5H19.5C20.8807 10.5 22 11.6193 22 13C22 14.3807 20.8807 15.5 19.5 15.5H18V17.2C18 18.8802 18 19.7202 17.673 20.362C17.3854 20.9265 16.9265 21.3854 16.362 21.673C15.7202 22 14.8802 22 13.2 22H12.5V20.25C12.5 19.0074 11.4926 18 10.25 18C9.00736 18 8 19.0074 8 20.25V22H6.8C5.11984 22 4.27976 22 3.63803 21.673C3.07354 21.3854 2.6146 20.9265 2.32698 20.362C2 19.7202 2 18.8802 2 17.2V15.5H3.5C4.88071 15.5 6 14.3807 6 13C6 11.6193 4.88071 10.5 3.5 10.5H2C2 9.10218 2 8.40326 2.22836 7.85195C2.53284 7.11687 3.11687 6.53284 3.85195 6.22836C4.40326 6 5.10218 6 6.5 6H7.5V4.5Z"
          stroke="white"
          stroke-width="0.5"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-dasharray="0 1.5 0"
        />
      </svg>
    </div>
    <div class="grow flex gap-4 justify-center">
      <ul class="grow-0 mt-[50px] text-center space-y-4 text-2xl">
        <Step fadeUp><li>True move semantics in C++? Borrow checker?</li></Step>
        <Step fadeUp><li>Multiple returns in C++?</li></Step>
        <Step fadeUp><li>grep + clang-tidy + checks-from-json?</li></Step>
      </ul>
    </div>
  </Slide>

  <!-- How do you figure out what to match? -->

  <!-- cxxConstructExpr(has(cxxStdInitializerListExpr(hasDescendant(initListExpr()))),
                 hasDeclaration(cxxConstructorDecl(
                     ofClass(cxxRecordDecl(classTemplateSpecializationDecl(
                         hasName("::std::vector"),
                         hasTemplateArgument(0, templateArgument()))))))) -->

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
