#include "x.h"

#include <mutex>
#include <condition_variable>

class FooBar {
private:
  int n;
  std::mutex mu;
  std::condition_variable cv;
  int status = 0;

public:
  FooBar(int n) { this->n = n; }

  void foo(function<void()> printFoo) {

    for (int i = 0; i < n; i++) {
      std::unique_lock<std::mutex> lock(mu);
      cv.wait(lock, [this] { return status == 0; });
      // printFoo() outputs "foo". Do not change or remove this line.
      printFoo();
      status = 1;
      cv.notify_one();
    }
  }

  void bar(function<void()> printBar) {

    for (int i = 0; i < n; i++) {
      std::unique_lock<std::mutex> lock(mu);
      cv.wait(lock, [this] { return status == 1; });
      // printBar() outputs "bar". Do not change or remove this line.
      printBar();
      status = 0;
      cv.notify_one();
    }
  }
};
