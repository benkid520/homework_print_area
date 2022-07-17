

fn main() {
    //创建一个Rectangle 用变量r接收
    let r=Rectangle{
        height:8,
        width:9,
    };
    //调用函数 打印面积
    print_rectangle_area(&r);
}
//声明定义结构体 rectangle 宽高的数据类型定义 为由传入的 泛型决定
struct Rectangle<T>{
    width:T,
    height:T,
}
//声明定义trait Property 声明两个获取 高 宽成员变量的函数 返回泛型 T
trait Property<T> {
    fn get_height(&self)->&T;
    fn get_width(&self)->&T;
}
//为Rectangle 实现trait Property 定义两个获取 高 宽成员变量的函数 返回泛型 T
impl<T> Property<T> for Rectangle<T> {
    fn get_height(&self)->&T{
        &(self.height)
    }
    fn get_width(&self)->&T{
        &(self.width)
    }
}
//对函数实施泛型约束 只允许trait Property<u16> 的数据传入 
//打印传入数据的面积
fn print_rectangle_area<T:Property<u16>>(item:&T){
    println!("{}",item.get_height()*item.get_width());
}
//End

//泛型约束不允许 + Property<f32> 实施多个动态泛型trait的约束
//如果需要实现这样的效果应如何操作 希望点评的老师能解答下 感谢！
// fn print_rectangle_area<T:Property<u16> + Property<f32>>(item:&T){
//     println!("{}",item.get_height()*item.get_width());
// }
